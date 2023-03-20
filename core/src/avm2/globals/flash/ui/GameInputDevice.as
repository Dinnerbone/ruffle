package flash.ui {
import flash.utils.ByteArray;

public final class GameInputDevice {

        public native function get enabled():Boolean;
        public native function set enabled(enabled:Boolean):void;

        public native function get sampleInterval():int;
        public native function set sampleInterval(sampleInterval:int):void;

        public native function get id():String;

        public native function get name():String;

        public native function get numControls():int;

        public native function getCachedSamples(data:ByteArray, append:Boolean = false):int;

        //public native function getControlAt(i:int):GameInputControl;

        public native function startCachingSamples(numSamples:int, controls:Vector.<String>):void;

        public native function stopCachingSamples():void;
    }
}