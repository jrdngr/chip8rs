const PIXEL_SIZE = 20;
const PIXEL_ON_COLOR = "#000000";
const PIXEL_OFF_COLOR = "#FFFFFF";

export class Display {
    
    private readonly pixels: boolean[] = [];
    private readonly ctx: CanvasRenderingContext2D;
    
    constructor(private readonly width: number, 
                private readonly height: number, 
                canvas: HTMLCanvasElement)
    {
        this.ctx = canvas.getContext('2d');
        canvas.width = PIXEL_SIZE * width;
        canvas.height = PIXEL_SIZE * height;

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
        this.ctx.beginPath();
        this.ctx.fillStyle = isOn
            ? PIXEL_ON_COLOR
            : PIXEL_OFF_COLOR;
    
        this.ctx.fillRect(
            x * PIXEL_SIZE,
            y * PIXEL_SIZE,
            PIXEL_SIZE,
            PIXEL_SIZE
        );
    }

    private getIndex(x: number, y: number): number {
        return (y * this.width) + x;
    }
}