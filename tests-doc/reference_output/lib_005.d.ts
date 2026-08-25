/* tslint:disable */
/* eslint-disable */
declare namespace Color {
    export type Red = "Red";
    export type Blue = "Blue";
    export type Green = "Green";
    export type Rgb = { Rgb: [number, number, number] };
    export type Hsv = { Hsv: { hue: number; saturation: number; value: number } };
}

export type Color = Color.Red | Color.Blue | Color.Green | Color.Rgb | Color.Hsv;
