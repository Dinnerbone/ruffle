package flash.events {
    public final class GameInputEvent extends Event {
        public static const DEVICE_ADDED:String = "deviceAdded";
        public static const DEVICE_REMOVED:String = "deviceRemoved";
        public static const DEVICE_UNUSABLE:String = "deviceUnusable";

        public function GameInputEvent(type:String, bubbles:Boolean = false, cancelable:Boolean = false, device:* = null) {
            super(type, bubbles, cancelable);
        }
    }
}