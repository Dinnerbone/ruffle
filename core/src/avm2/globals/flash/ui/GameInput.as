package flash.ui {
    import flash.events.EventDispatcher;

    public final class GameInput extends EventDispatcher {
        public function GameInput() {
            init();
        }

        private native function init():void;

        public static native function get isSupported():Boolean;

        public static function get numDevices():int {
            return 0;
        }
    }
}