//! macOS notifications, including the actionable Snooze/Dismiss buttons that the
//! official Tauri plugin cannot provide on desktop (its Actions API is
//! mobile-only). Off macOS this whole module is a no-op.

use std::sync::OnceLock;

use block2::DynBlock;
use objc2::rc::Retained;
use objc2::runtime::{NSObject, NSObjectProtocol, ProtocolObject};
use objc2::{AllocAnyThread, DefinedClass, define_class, msg_send};
use objc2_foundation::{NSArray, NSBundle, NSSet, NSString};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNMutableNotificationContent, UNNotification, UNNotificationAction,
    UNNotificationActionOptions, UNNotificationCategory, UNNotificationCategoryOptions,
    UNNotificationPresentationOptions, UNNotificationRequest, UNNotificationResponse,
    UNTimeIntervalNotificationTrigger, UNUserNotificationCenter, UNUserNotificationCenterDelegate,
};

use super::{ACTION_DISMISS, ACTION_SNOOZE, NotificationActionHandler};

/// The category every reminder is posted under, so the buttons appear.
const CATEGORY: &str = "work-dashboard.reminder";

/// `UNUserNotificationCenter.current()` raises when the process has no bundle
/// identifier — which is every `tauri dev` run and any bare binary. So this
/// check is a hard gate, not a nicety: without it the app aborts at startup
/// rather than simply not notifying.
pub fn available() -> bool {
    static AVAILABLE: OnceLock<bool> = OnceLock::new();
    *AVAILABLE.get_or_init(|| {
        let bundled = NSBundle::mainBundle().bundleIdentifier().is_some();
        if !bundled {
            eprintln!(
                "notifications unavailable: no bundle identifier (run a packaged build, not `tauri dev`)"
            );
        }
        bundled
    })
}

fn center() -> Retained<UNUserNotificationCenter> {
    UNUserNotificationCenter::currentNotificationCenter()
}

struct DelegateIvars {
    handler: NotificationActionHandler,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements, and the delegate holds
    // only a thread-safe closure.
    #[unsafe(super(NSObject))]
    #[name = "WorkDashboardNotificationDelegate"]
    #[ivars = DelegateIvars]
    struct NotificationDelegate;

    unsafe impl NSObjectProtocol for NotificationDelegate {}

    unsafe impl UNUserNotificationCenterDelegate for NotificationDelegate {
        /// Without this, macOS suppresses the banner while our own app is
        /// frontmost — which for a desktop app is most of the time.
        #[unsafe(method(userNotificationCenter:willPresentNotification:withCompletionHandler:))]
        fn will_present(
            &self,
            _center: &UNUserNotificationCenter,
            _notification: &UNNotification,
            completion_handler: &DynBlock<dyn Fn(UNNotificationPresentationOptions)>,
        ) {
            completion_handler.call((UNNotificationPresentationOptions::Banner
                | UNNotificationPresentationOptions::Sound,));
        }

        #[unsafe(method(userNotificationCenter:didReceiveNotificationResponse:withCompletionHandler:))]
        fn did_receive(
            &self,
            _center: &UNUserNotificationCenter,
            response: &UNNotificationResponse,
            completion_handler: &DynBlock<dyn Fn()>,
        ) {
            let action = response.actionIdentifier().to_string();
            let identifier = response.notification().request().identifier().to_string();
            (self.ivars().handler)(&action, &identifier);
            completion_handler.call(());
        }
    }
);

impl NotificationDelegate {
    fn new(handler: NotificationActionHandler) -> Retained<Self> {
        let this = Self::alloc().set_ivars(DelegateIvars { handler });
        unsafe { msg_send![super(this), init] }
    }
}

/// The center's `delegate` is a *weak* property, so the delegate has to be kept
/// alive here or every callback silently stops arriving.
static DELEGATE: OnceLock<Retained<NotificationDelegate>> = OnceLock::new();

/// Installs the delegate and registers the action category. Idempotent.
pub fn install(handler: NotificationActionHandler) {
    if !available() {
        return;
    }

    DELEGATE.get_or_init(|| {
        let delegate = NotificationDelegate::new(handler);
        center().setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        delegate
    });

    let snooze = UNNotificationAction::actionWithIdentifier_title_options(
        &NSString::from_str(ACTION_SNOOZE),
        &NSString::from_str("Snooze 10 minutes"),
        UNNotificationActionOptions::empty(),
    );
    let dismiss = UNNotificationAction::actionWithIdentifier_title_options(
        &NSString::from_str(ACTION_DISMISS),
        &NSString::from_str("Dismiss"),
        UNNotificationActionOptions::empty(),
    );
    let actions = NSArray::from_retained_slice(&[snooze, dismiss]);
    let category = UNNotificationCategory::categoryWithIdentifier_actions_intentIdentifiers_options(
        &NSString::from_str(CATEGORY),
        &actions,
        &NSArray::from_slice(&[]),
        UNNotificationCategoryOptions::empty(),
    );
    center().setNotificationCategories(&NSSet::from_retained_slice(&[category]));
}

/// Asks for permission. The completion runs on an arbitrary queue, so the answer
/// is reported through the handler rather than returned.
pub fn request_authorization(handler: Box<dyn Fn(bool) + Send + Sync>) {
    if !available() {
        return;
    }

    let block = block2::RcBlock::new(move |granted: objc2::runtime::Bool, _error| {
        handler(granted.as_bool());
    });
    center().requestAuthorizationWithOptions_completionHandler(
        UNAuthorizationOptions::Alert | UNAuthorizationOptions::Sound,
        &block,
    );
}

/// Posts a notification `after_seconds` from now, carrying the action buttons.
pub fn post(identifier: &str, title: &str, body: &str, after_seconds: f64) {
    if !available() {
        return;
    }

    let content = UNMutableNotificationContent::new();
    content.setTitle(&NSString::from_str(title));
    content.setBody(&NSString::from_str(body));
    content.setCategoryIdentifier(&NSString::from_str(CATEGORY));

    // The trigger raises on a non-positive interval, and a reminder whose time
    // has already passed should still be announced rather than dropped.
    let delay = after_seconds.max(1.0);
    let trigger = UNTimeIntervalNotificationTrigger::triggerWithTimeInterval_repeats(delay, false);
    let request = UNNotificationRequest::requestWithIdentifier_content_trigger(
        &NSString::from_str(identifier),
        &content,
        Some(&trigger),
    );
    center().addNotificationRequest_withCompletionHandler(&request, None);
}

/// Drops pending requests. Identifiers the system doesn't know about are ignored,
/// so this is safe to call for anything no longer wanted.
pub fn cancel(identifiers: &[String]) {
    if !available() || identifiers.is_empty() {
        return;
    }

    let names: Vec<Retained<NSString>> = identifiers
        .iter()
        .map(|identifier| NSString::from_str(identifier))
        .collect();
    center()
        .removePendingNotificationRequestsWithIdentifiers(&NSArray::from_retained_slice(&names));
}
