/* tslint:disable */
/* eslint-disable */
export interface AbiEnvelope<T> {
    data: T;
}

export interface Arrays {
    small: [number, number];
    big: number[];
}

export interface Defaulted {
    opt: number | null;
    map: Record<string, number>;
    big: number;
}

export interface Envelope<T> {
    data: T;
}

export interface Payload {
    name: string;
}


export function abi_roundtrip(v: AbiEnvelope<number>): AbiEnvelope<number>;

export function arg_array_big(v: Envelope<number[]>): void;

export function arg_array_small(v: Envelope<[number, number]>): void;

export function arg_map(v: Envelope<Record<string, number>>): void;

export function arg_option(v: Envelope<(number | null)>): void;

export function arg_result(v: Envelope<({ Ok: number } | { Err: string })>): void;

export function arg_tuple(v: Envelope<[number, string]>): void;

export function arg_u64(v: Envelope<number>): void;

export function arg_vec(v: Envelope<number[]>): void;

export function generic_arg(v: Envelope<Payload>): void;

export function generic_builtin(v: Envelope<number>): void;

export function generic_nested(v: Envelope<Envelope<Payload>>): void;

export function generic_return(v: Envelope<Payload>): Envelope<Payload>;

export function plain_arg(v: Payload): void;

export function plain_return(v: Payload): Payload;
