import flash.display.BitmapData;
import flash.filters.BlurFilter;
import flash.geom.Point;
import flash.geom.Rectangle;
import flash.geom.Matrix;
import flash.geom.ColorTransform;

var disposedBmd;
var validBmd;
var rect = new Rectangle(0, 0, 10, 10);
var rectAsObj = {x: 0, y: 0, width: 10, height: 10};
var rectWithoutWidth = {x: 0, y: 0, height: 10};
var zeroSizedRect = new Rectangle(5, 5, 0, 0);
var point = new Point(3, 1);
var pointOutsideBmd = new Point(100, 100);
var matrix = new Matrix();
var matrixAsObj = {a:1,b:1,c:1,d:1,tx:1,ty:1};
var matrixWithoutTx = {a:1,b:1,c:1,d:1,ty:1};
var colorTransform = new ColorTransform(0, 0, 1, 1, 0, 0, 255, 0);
var blurFilter = new BlurFilter(30, 30, 2);

function main() {
    constructWithDifferentArgs("BitmapData", 50, 60, true, 0xAABBCCDD);
    var createObject = function() { return new BitmapData(50, 60, true, 0xAABBCCDD); };
    var createDisposedObject = function() { var o = createObject(); o.dispose(); return o; };
    validBmd = createObject();
    disposedBmd = createDisposedObject();

    var functionsAndArgs = [
        ["getPixel", 1, 2],
        ["getPixel32", 1, 2],
        ["setPixel", 1, 2, 0x12345678],
        ["setPixel32", 1, 2, 0x12345678],
        ["copyChannel", validBmd, rect, point, 3, 1],
        ["fillRect", rect, 0x12345678],
        ["floodFill", 1, 2, 0x12345678],
        ["noise", 128, 0, 255, 1, true],
        ["draw", validBmd, matrix, colorTransform, "normal", rect, true],
        ["applyFilter", validBmd, rect, point, blurFilter],
        ["colorTransform", rect, colorTransform],
        ["getColorBoundsRect", 0x00FFFFFF, 0x00FF0000, true]
        // ["generateFilterRect", rect, blurFilter]
    ];

    for (var i = 0; i < functionsAndArgs.length; i++) {
        callWithDifferentArgs(createObject, functionsAndArgs[i][0], functionsAndArgs[i].slice(1));
    }
    callWithSpecificArgs(createObject, "getColorBoundsRect", [0, 0, true]);
    callWithSpecificArgs(createObject, "getColorBoundsRect", [0, 1, true]);
    
    trace("");
    trace("////// disposed from here")
    trace("");

    for (var i = 0; i < functionsAndArgs.length; i++) {
        callWithSpecificArgs(createDisposedObject, functionsAndArgs[i][0], functionsAndArgs[i].slice(1));
    }
}

function generateBadArguments(good) {
    var result = [null, undefined, good, {}];
    if (good == validBmd) {
        result.push(disposedBmd);
    }
    if (good == rect) {
        result.push(zeroSizedRect);
        result.push(rectAsObj);
        result.push(rectWithoutWidth);
    }
    if (good == matrix) {
        result.push(matrixAsObj);
        result.push(matrixWithoutTx);
    }
    if (good == point) {
        result.push(pointOutsideBmd);
    }
    return result;
}

function constructWithDifferentArgs(className) {
    var allArgsToTest = generateArgSets(arguments.slice(1));
    for (var i = 0; i < allArgsToTest.length; i++) {
        var argStr = "";
        for (var j = 0; j < allArgsToTest[i].length; j++) {
            if (j > 0) {
                argStr += ", ";
            }
            argStr += valueToString(allArgsToTest[i][j]);
        }
        trace("// new " + className + "(" + argStr + ")");
        var clas = _global.flash.display[className];
        var f = constructWithArgs(clas, allArgsToTest[i]);
        trace(valueToString(f));
        if (!(f instanceof clas)) {
            trace("NOT AN INSTANCEOF!");
        }
        trace("");
    }
}

function callWithDifferentArgs(createObject, functionName, knownGoodArguments) {
    var allArgsToTest = generateArgSets(knownGoodArguments);
    for (var i = 0; i < allArgsToTest.length; i++) {
        var argStr = "";
        for (var j = 0; j < allArgsToTest[i].length; j++) {
            if (j > 0) {
                argStr += ", ";
            }
            argStr += valueToString(allArgsToTest[i][j]);
        }
        trace("// " + functionName + "(" + argStr + ")");
        var object = createObject();
        var f = callWithArgs(object, functionName, allArgsToTest[i]);
        trace(valueToString(f));
        trace("");
    }
}

function callWithSpecificArgs(createObject, functionName, args) {
    var argStr = "";
    for (var j = 0; j < args.length; j++) {
        if (j > 0) {
            argStr += ", ";
        }
        argStr += valueToString(args[j]);
    }
    trace("// " + functionName + "(" + argStr + ")");
    var object = createObject();
    var f = callWithArgs(object, functionName, args);
    trace(valueToString(f));
    trace("");
}

function callWithArgs(object, func, args) {
    switch (args.length) {
        case 0: return object[func]();
        case 1: return object[func](args[0]);
        case 2: return object[func](args[0], args[1]);
        case 3: return object[func](args[0], args[1], args[2]);
        case 4: return object[func](args[0], args[1], args[2], args[3]);
        case 5: return object[func](args[0], args[1], args[2], args[3], args[4]);
        case 6: return object[func](args[0], args[1], args[2], args[3], args[4], args[5]);
        case 7: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
        case 8: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
        case 9: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
        case 10: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
        case 11: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]);
        case 12: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]);
        case 13: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12]);
        case 14: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13]);
        case 15: return object[func](args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13], args[14]);
    }
    trace("INVALID TEST: Too many arguments!");
    return null;
}

function constructWithArgs(cls, args) {
    switch (args.length) {
        case 0: return new cls();
        case 1: return new cls(args[0]);
        case 2: return new cls(args[0], args[1]);
        case 3: return new cls(args[0], args[1], args[2]);
        case 4: return new cls(args[0], args[1], args[2], args[3]);
        case 5: return new cls(args[0], args[1], args[2], args[3], args[4]);
        case 6: return new cls(args[0], args[1], args[2], args[3], args[4], args[5]);
        case 7: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6]);
        case 8: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]);
        case 9: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]);
        case 10: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]);
        case 11: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]);
        case 12: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]);
        case 13: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12]);
        case 14: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13]);
        case 15: return new cls(args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13], args[14]);
    }
    trace("INVALID TEST: Too many arguments!");
    return null;
}

function generateArgSets(knownGoodArguments) {
    var results = [];

    // Partial sets (e.g. [], [1], [1, 2], ...)
    for (var i = 0; i <= knownGoodArguments.length; i++) {
        results.push(knownGoodArguments.slice(0, i));
    }

    // Replace one argument at a time with each of its specific bad variants
    for (var i = 0; i < knownGoodArguments.length; i++) {
        var bads = generateBadArguments(knownGoodArguments[i]);
        for (var b = 0; b < bads.length; b++) {
            var variant = knownGoodArguments.concat();
            variant[i] = bads[b];
            results.push(variant);
        }
    }


    // Deduplicate
    var unique = [];
    for (var i = 0; i < results.length; i++) {
        var a = results[i];
        var found = false;
        for (var j = 0; j < unique.length; j++) {
            if (arraysEqual(a, unique[j])) { found = true; break; }
        }
        if (!found) unique.push(a);
    }

    return unique;
}

function arraysEqual(a, b) {
    if (a.length != b.length) return false;
    for (var i = 0; i < a.length; i++) {
        if (a[i] != b[i]) return false;
    }
    return true;
}

function valueToString(v) {
    if (v === disposedBmd) {
        return "disposedBmd";
    }
    if (v === validBmd) {
        return "validBmd";
    }
    if (v === rect) {
        return "rect";
    }
    if (v === point) {
        return "point";
    }
    if (v === zeroSizedRect) {
        return "zeroSizedRect";
    }
    if (v === pointOutsideBmd) {
        return "pointOutsideBmd";
    }
    if (v === rectAsObj) {
        return "rectAsObj";
    }
    if (v === rectWithoutWidth) {
        return "rectWithoutWidth";
    }
    if (v === colorTransform) {
        return "colorTransform";
    }
    if (v === matrix) {
        return "matrix";
    }
    if (v === matrixAsObj) {
        return "matrixAsObj";
    }
    if (v === matrixWithoutTx) {
        return "matrixWithoutTx";
    }
    if (v === blurFilter) {
        return "blurFilter";
    }
    if (v instanceof Array) {
        var result = "";
        for (var i = 0; i < result.length; i++) {
            if (i > 0) {
                result += ", ";
            }
            result += valueToString(v);
        }
        return result;
    }
    if (typeof v == "string") {
        var result = "";
        for (var i= 0; i < v.length; i++) {
            var c = v.charAt(i);
            if (c == "\\") result += "\\\\";
            else if (c == "\"") result += "\\\"";
            else if (c == "\n") result += "\\n";
            else if (c == "\r") result += "\\r";
            else if (c == "\t") result += "\\t";
            else result += c;
        }
        return "\"" + result + "\"";
    }
    if (typeof v == "object") {
        var props = [];
        if (v instanceof Rectangle) {
            props.push("width");
            props.push("height");
            props.push("x");
            props.push("y");
        } else {
            for (var prop in v) {
                if (typeof v[prop] !== "function") {
                    props.push(prop);
                }
            }
        }
        props.sort();
        var str = "";
        for (var i = 0; i < props.length; i++) {
            var prop = props[i];
            if (str != "") str += ", ";
            str += prop + "=" + valueToString(v[prop]);
        }
        return "{ " + str + " }";
    }
    return "" + v;
}


main();