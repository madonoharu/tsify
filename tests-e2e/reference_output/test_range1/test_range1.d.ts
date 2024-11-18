/* tslint:disable */
/* eslint-disable */
/**
 * @param {Range} _range
 */
export function consume(_range: Range): void;
/**
 * @returns {Range}
 */
export function into_js(): Range;
/**
 * @param {(Range)[]} _ranges
 */
export function consume_vector(_ranges: (Range)[]): void;
/**
 * @returns {(Range)[]}
 */
export function vector_into_js(): (Range)[];
export interface Range {
    foo: number;
    bar: string;
}

export interface A {
    range: Range;
}

