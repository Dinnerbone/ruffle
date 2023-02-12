// The initial version of this file was autogenerated from the official AS3 reference at
// https://help.adobe.com/en_US/FlashPlatform/reference/actionscript/3/flash/events/GestureEvent.html
// by https://github.com/golfinq/ActionScript_Event_Builder
// It won't be regenerated in the future, so feel free to edit and/or fix
package flash.events {

import flash.utils.ByteArray;
import flash.display.InteractiveObject;

public class TouchEvent extends Event {
    public static const PROXIMITY_BEGIN: String = "proximityBegin"; // [static] Defines the value of the type property of a PROXIMITY_BEGIN touch event object.
    public static const PROXIMITY_END: String = "proximityEnd"; // [static] Defines the value of the type property of a PROXIMITY_END touch event object.
    public static const PROXIMITY_MOVE: String = "proximityMove"; // [static] Defines the value of the type property of a PROXIMITY_MOVE touch event object.
    public static const PROXIMITY_OUT: String = "proximityOut"; // [static] Defines the value of the type property of a PROXIMITY_OUT touch event object.
    public static const PROXIMITY_OVER: String = "proximityOver"; // [static] Defines the value of the type property of a PROXIMITY_OVER touch event object.
    public static const PROXIMITY_ROLL_OUT: String = "proximityRollOut"; // [static] Defines the value of the type property of a PROXIMITY_ROLL_OUT touch event object.
    public static const PROXIMITY_ROLL_OVER: String = "proximityRollOver"; // [static] Defines the value of the type property of a PROXIMITY_ROLL_OVER touch event object.
    public static const TOUCH_BEGIN: String = "touchBegin"; // [static] Defines the value of the type property of a TOUCH_BEGIN touch event object.
    public static const TOUCH_END: String = "touchEnd"; // [static] Defines the value of the type property of a TOUCH_END touch event object.
    public static const TOUCH_MOVE: String = "touchMove"; // [static] Defines the value of the type property of a TOUCH_MOVE touch event object.
    public static const TOUCH_OUT: String = "touchOut"; // [static] Defines the value of the type property of a TOUCH_OUT touch event object.
    public static const TOUCH_OVER: String = "touchOver"; // [static] Defines the value of the type property of a TOUCH_OVER touch event object.
    public static const TOUCH_ROLL_OUT: String = "touchRollOut"; // [static] Defines the value of the type property of a TOUCH_ROLL_OUT touch event object.
    public static const TOUCH_ROLL_OVER: String = "touchRollOver"; // [static] Defines the value of the type property of a TOUCH_ROLL_OVER touch event object.
    public static const TOUCH_TAP: String = "touchTap"; // [static] Defines the value of the type property of a TOUCH_TAP touch event object.

    public var touchPointID: int; // A unique identification number (as an int) assigned to the touch point.
    public var isPrimaryTouchPoint: Boolean; // Indicates whether the first point of contact is mapped to mouse events.
    public var localX: Number; // The horizontal coordinate at which the event occurred relative to the containing sprite.
    public var localY: Number; // The vertical coordinate at which the event occurred relative to the containing sprite.
    public var sizeX: Number; // Width of the contact area.
    public var sizeY: Number; // Height of the contact area.
    public var pressure: Number; // A value between 0.0 and 1.0 indicating force of the contact with the device.
    public var relatedObject: InteractiveObject; // A reference to a display list object that is related to the event.
    public var ctrlKey: Boolean; // On Windows or Linux, indicates whether the Ctrl key is active (true) or inactive (false).
    public var altKey: Boolean; // Indicates whether the Alt key is active (true) or inactive (false).
    public var shiftKey: Boolean; // Indicates whether the Shift key is active (true) or inactive (false).
    public var commandKey: Boolean; // Indicates whether the command key is activated (Mac only).
    public var controlKey: Boolean; // Indicates whether the Control key is activated on Mac and whether the Ctrl key is activated on Windows or Linux.
    public var timestamp: Number; // Reports the time of the event in relative milliseconds.
    public var touchIntent: String; // Reports whether the touch was generated by the primary or the eraser end of a stylus.
    public var isTouchPointCanceled: Boolean; // Reports that this touch input sequence was canceled by the operating system.
    public var isRelatedObjectInaccessible: Boolean; // If true, the relatedObject property is set to null for reasons related to security sandboxes.
    private var _stageX: Number; // [read-only] The horizontal coordinate at which the event occurred in global Stage coordinates.
    private var _stageY: Number; // [read-only] The vertical coordinate at which the event occurred in global Stage coordinates.

    public function TouchEvent(type: String, bubbles: Boolean = true, cancelable: Boolean = false, touchPointID: int = 0,
                               isPrimaryTouchPoint: Boolean = false, localX: Number = NaN, localY: Number = NaN,
                               sizeX: Number = NaN, sizeY: Number = NaN, pressure: Number = NaN,
                               relatedObject: InteractiveObject = null, ctrlKey: Boolean = false,
                               altKey: Boolean = false, shiftKey: Boolean = false, commandKey: Boolean = false,
                               controlKey: Boolean = false, timestamp: Number = NaN, touchIntent: String = "unknown",
                               samples: ByteArray = null, isTouchPointCanceled: Boolean = false) {
        super(type, bubbles, cancelable, samples);
        this.touchPointID = touchPointID;
        this.isPrimaryTouchPoint = isPrimaryTouchPoint;
        this.localX = localX;
        this.localY = localY;
        this.sizeX = sizeX;
        this.sizeY = sizeY;
        this.pressure = pressure;
        this.relatedObject = relatedObject;
        this.ctrlKey = ctrlKey;
        this.altKey = altKey;
        this.shiftKey = shiftKey;
        this.commandKey = commandKey;
        this.controlKey = controlKey;
        this.timestamp = timestamp;
        this.touchIntent = touchIntent;
        this.isTouchPointCanceled = isTouchPointCanceled;
    }


    // [override] Creates a copy of the TouchEvent object and sets the value of each property to match that of the original.
    override public function clone(): Event {
        return new TouchEvent(this.type, this.bubbles, this.cancelable, this.touchPointID, this.isPrimaryTouchPoint,
            this.localX, this.localY, this.sizeX, this.sizeY, this.pressure, this.relatedObject, this.ctrlKey,
            this.altKey, this.shiftKey, this.commandKey, this.controlKey, this.timestamp, this.touchIntent,
            this.isTouchPointCanceled);
    }

    // Updates the specified ByteArray object with the high-frequency data points for a multi-point touch event.
    public function getSamples(buffer: ByteArray, append: Boolean = false): uint {
        stub_method("flash.events.TouchEvent", "getSamples");
        return 0;
    }

    // Reports that the hardware button at the specified index is pressed.
    public function isToolButtonDown(index: int): Boolean {
        stub_method("flash.events.TouchEvent", "isToolButtonDown");
        return false;
    }

    // [override] Returns a string that contains all the properties of the TouchEvent object.
    override public function toString(): String {
        return this.formatToString("TouchEvent", "type", "bubbles", "cancelable", "eventPhase", "touchPointID",
            "isPrimaryTouchPoint", "localX", "localY", "sizeX", "sizeY", "pressure", "relatedObject", "ctrlKey",
            "altKey", "shiftKey", "commandKey", "controlKey", "timestamp", "touchIntent", "isTouchPointCanceled",
            "isRelatedObjectInaccessible", "stageX", "stageY");
    }

    // Instructs Flash Player or Adobe AIR to render after processing of this event completes, if the display list has been modified.
    public function updateAfterEvent(): void {
        stub_method("flash.events.TouchEvent", "updateAfterEvent");
    }

    public function get stageX(): Number {
        return this._stageX;
    }

    public function get stageY(): Number {
        return this._stageY;
    }
}
}