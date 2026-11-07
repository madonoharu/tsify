/* tslint:disable */
/* eslint-disable */
export interface RenamedStruct {
    a: number;
}

export type C = number[];

export type D = string;

export type PreB = number[];

export type RenamedAlias = number[];

export type RenamedEnum = { V: number };

export type TsBaz = TsFoo<number>[];

export type TsFoo<T> = T;
