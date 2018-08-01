import * as PIXI from 'pixi.js';

const PIXEL_SIZE = 15;
const PIXEL_ON_COLOR = 0x000000;

export class Display {
    private readonly width: number;
    private readonly height: number;
    private readonly pixels: PIXI.Graphics[] = [];
    
    private readonly app: PIXI.Application;

    constructor(width: number, height: number)
    {
        this.width = width;
        this.height = height;

        this.app = new PIXI.Application({  
                width: width * PIXEL_SIZE,
                height: height * PIXEL_SIZE,
                backgroundColor: 0xFFFFFF, 
            });
        const screenDiv = document.getElementById("screen");
        screenDiv.appendChild(this.app.view);

        for (let i = 0, max = width * height; i < max; i++) {
            const {x, y} = this.getCoordinates(i);
            const pixel = this.drawPixel(x, y)
            this.pixels.push(pixel);
            this.app.stage.addChild(pixel);
        }
    }

    public setPixel(x: number, y: number) {
        const index = this.getIndex(x, y);
        this.pixels[index].visible = !this.pixels[index].visible;
    }

    public clear() {
        this.pixels.forEach(pixel => {
            pixel.fillAlpha = 0;
        });
    }

    private drawPixel(x: number, y: number): PIXI.Graphics {
        let rectangle = new PIXI.Graphics();
        rectangle.beginFill(0x000000);
        rectangle.drawRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
        rectangle.endFill();
        rectangle.visible = false;
        return rectangle;
    }

    private getIndex(x: number, y: number): number {
        return (y * this.width) + x;
    }

    private getCoordinates(index: number): { x: number, y: number } {
        const x = index % this.width;
        const y = Math.floor(index / this.width);
        return { x, y };
    }
}