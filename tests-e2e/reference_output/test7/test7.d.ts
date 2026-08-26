/* tslint:disable */
/* eslint-disable */
/**
 * A parameter no field mentions is not part of the declaration, so it is not
 * part of the name either.
 */
export interface Tagged {
    id: number;
}

export interface Pair<A, B> {
    left: A;
    right: B;
}

export interface Response<T> {
    data: T;
    ok: boolean;
}

export interface UserInfo {
    id: number;
    name: string;
}

export type Outcome<T> = { Done: T } | { Failed: string };


export function builtin_argument(): Response<string[]>;

export function generic_enum(): Outcome<UserInfo>;

export function get_user(): Response<UserInfo>;

export function map_argument(): Response<Record<string, number>>;

export function nested(): Response<Response<UserInfo>>;

export function optional_argument(): Response<(UserInfo | null)>;

export function put_user(response: Response<UserInfo>): boolean;

/**
 * The same thing through `Ts`, which is the way to cross the ABI without the
 * leak `into_wasm_abi`/`from_wasm_abi` have.
 */
export function round_trip(response: Response<UserInfo>): Response<UserInfo>;

export function several_arguments(): Response<Pair<number, string>>;

export function undeclared_parameter(): Tagged;
