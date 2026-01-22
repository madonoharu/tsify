/* tslint:disable */
/* eslint-disable */
export interface A {
    range: Range;
}

export interface Range {
    foo: number;
    bar: string;
}


export function consume(_range: Range): void;

export function consume_vector(_ranges: Range[]): void;

export function into_js(): Range;

export function vector_into_js(): Range[];
