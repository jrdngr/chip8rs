import * as PIXI from 'pixi.js';

const PIXEL_SIZE = 15;
const PIXEL_ON_COLOR = 0x000000;
const PIXEL_OFF_COLOR = 0xFFFFFF;

export class Display {
    private readonly width: number;
    private readonly height: number;
    private readonly pixels: boolean[] = [];
    
    private readonly app: PIXI.Application;

    constructor(width: number, height: number)
    {
        this.width = PIXEL_SIZE * width;
        this.height = PIXEL_SIZE * height;

        this.app = new PIXI.Application(
            { backgroundColor: 0xFFFFFF, }
        );
        const screenDiv = document.getElementById("screen");
        screenDiv.appendChild(this.app.view);

        for (let i = 0, max = width * height; i < max; i++) {
            this.pixels.push(false);
        }
    }

    public setPixel(x: number, y: number) {
        const index = this.getIndex(x, y);
        const current = this.pixels[index];
        if (!current) {
            this.pixels[index] = true;
        } else {
            this.pixels[index] = false;
        }
        
        this.drawPixel(x, y, this.pixels[index]);
    }

    public clear() {
        for (let row = 0; row < this.width; row++) {
            for (let col = 0; col < this.height; col++) {
                let index = this.getIndex(row, col);
                this.pixels[index] = false;
                this.drawPixel(row, col, false);
            }
        }
    }

    private drawPixel(x: number, y: number, isOn: boolean) {
        let rectangle = new PIXI.Graphics();
        rectangle.beginFill(isOn ? PIXEL_ON_COLOR : PIXEL_OFF_COLOR);
        rectangle.drawRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
        rectangle.endFill();
        this.app.stage.addChild(rectangle);
    }

    private getIndex(x: number, y: number): number {
        return (y * this.width) + x;
    }
}