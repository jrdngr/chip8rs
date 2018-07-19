const PIXEL_SIZE = 5;
const PIXEL_ON_COLOR = "#000000";
const PIXEL_OFF_COLOR = "#FFFFFF";

export class Display {
    
    private readonly pixels: boolean[] = [];
    private readonly ctx: CanvasRenderingContext2D;
    
    constructor(private readonly width: number, 
                private readonly height: number, 
                private readonly canvas: HTMLCanvasElement) 
    {
        this.ctx = canvas.getContext('2d');
        canvas.width = PIXEL_SIZE * width;
        canvas.height = PIXEL_SIZE * height;

        for (let i = 0, max = width * height; i < max; i++) {
            this.pixels.push(false);
        }
    }

    public setPixel(x: number, y: number) {
        const index =this.getIndex(x, y);
        const current = this.pixels[index];
        if (!current) {
            this.pixels[index] = true;
        } else {
            this.pixels[index] = false;
        }
    }

    public clear() {
        this.pixels.forEach(pixel => pixel = false);
    }

    public draw() {
        this.ctx.beginPath();
      
        for (let row = 0; row < this.height; row++) {
          for (let col = 0; col < this.width; col++) {
            const index = this.getIndex(row, col);
      
            this.ctx.fillStyle = this.pixels[index]
              ? PIXEL_OFF_COLOR
              : PIXEL_ON_COLOR;
      
            this.ctx.fillRect(
              col * PIXEL_SIZE,
              row * PIXEL_SIZE,
              PIXEL_SIZE,
              PIXEL_SIZE
            );
          }
        }
    }

    private getIndex(x: number, y: number): number {
        return (y * this.width) + x;
    }
}