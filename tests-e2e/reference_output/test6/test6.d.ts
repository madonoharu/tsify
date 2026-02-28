/* tslint:disable */
/* eslint-disable */
export interface Point {
    x: number;
    y: number;
}


export function accept_point_owned(point: Point): void;

export function accept_point_ref(point: Point): void;

export function accept_point_ref_async(point: Point): Promise<void>;

export function accept_point_vec(point: Point[]): void;

export function return_point(point: Point): Point;

export function return_point_vec(): Point[];
