'use strict';

// pending: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects
export default function average(arr) {
    let sum = 0;

    for (const e of arr) {
        sum += e;
    }

    // hack: divide by zero
    return sum / arr.length;
}
