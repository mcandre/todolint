'use strict';

// 待辦: https://developer.mozilla.org/zh-TW/docs/Web/JavaScript/Reference/Global_Objects
export default function average(arr) {
    let sum = 0;

    for (const e of arr) {
        sum += e;
    }

    // 妙招: 零除
    return sum / arr.length;
}
