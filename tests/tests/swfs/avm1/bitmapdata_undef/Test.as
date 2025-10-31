import flash.display.BitmapData;

function main() {
    constructWithDifferentArgs("BitmapData", [50, NaN], [60, -1], [true], [0xAABBCCDD]);
    var createObject = function() { return new BitmapData(50, 60, true, 0xAABBCCDD); };

    callWithDifferentArgs(createObject, "getPixel", [NaN, 1], [2, -1, 1.5]);
}

function constructWithDifferentArgs(className) {
    var allArgsToTest = cartesianProduct(expandPossibleArgs(arguments.slice(1)));
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

function callWithDifferentArgs(createObject, functionName) {
    var allArgsToTest = cartesianProduct(expandPossibleArgs(arguments.slice(2)));
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

function expandPossibleArgs(validArguments) {
    var result = [];

    for (var i = 0; i < validArguments.length; i++) {
        var args = validArguments[i];
        args.push(null);
        args.push(undefined);
        result.push(args);
    }

    return result;
}


function cartesianProduct(arr) {
    var result = [[]];

    for (var i = 0; i < arr.length; i++) {
        var temp = [];
        for (var j = 0; j < result.length; j++) {
            for (var k = 0; k < arr[i].length; k++) {
                // Not directly concat(...) because that's overloaded for arrays, which is awkward
                var args = result[j].concat();
                args.push(arr[i][k]);
                temp.push(args);
            }
        }
        result = temp;
    }

    return result;
}

function valueToString(v) {
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
        for (var prop in obj) {
            if (typeof obj[prop] !== "function") {
                props.push(prop);
            }
        }
        props.sort();
        var str = "";
        for (var i = 0; i < props.length; i++) {
            var prop = props[i];
            if (str != "") str += ", ";
            str += prop + "=" + valueToString(obj[prop]);
        }
        trace("{ " + str + " }");
    }
    return "" + v;
}


main();