/* tslint:disable */
/* eslint-disable */
export type Bar = Foo<[number, number][]>;

export type Foo<T> = T;


export function returns_bar(): Foo;
