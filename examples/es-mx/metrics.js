'use strict';

// pendiente: https://developer.mozilla.org/es/docs/Web/JavaScript/Reference/Global_Objects
export default function average(arr) {
    let sum = 0;

    for (const e of arr) {
        sum += e;
    }

    // truco: dividir por cero"
    return sum / arr.length;
}
