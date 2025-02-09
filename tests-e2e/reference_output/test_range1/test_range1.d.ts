/* tslint:disable */
/* eslint-disable */
export function consume(_range: Range): void;
export function into_js(): Range;
export function consume_vector(_ranges: Range[]): void;
export function vector_into_js(): Range[];
export interface Range {
    foo: number;
    bar: string;
}

export interface A {
    range: Range;
}

