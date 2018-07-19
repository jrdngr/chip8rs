var PIXEL_SIZE = 5;
var PIXEL_ON_COLOR = "#000000";
var PIXEL_OFF_COLOR = "#FFFFFF";
var Display = /** @class */ (function () {
    function Display(width, height, canvas) {
        this.width = width;
        this.height = height;
        this.canvas = canvas;
        this.pixels = [];
        this.ctx = canvas.getContext('2d');
        canvas.width = PIXEL_SIZE * width;
        canvas.height = PIXEL_SIZE * height;
        for (var i = 0, max = width * height; i < max; i++) {
            this.pixels.push(false);
        }
    }
    Display.prototype.setPixel = function (x, y) {
        var index = this.getIndex(x, y);
        var current = this.pixels[index];
        if (!current) {
            this.pixels[index] = true;
        }
        else {
            this.pixels[index] = false;
        }
    };
    Display.prototype.clear = function () {
        this.pixels.forEach(function (pixel) { return pixel = false; });
    };
    Display.prototype.draw = function () {
        this.ctx.beginPath();
        for (var row = 0; row < this.height; row++) {
            for (var col = 0; col < this.width; col++) {
                var index = this.getIndex(row, col);
                this.ctx.fillStyle = this.pixels[index]
                    ? PIXEL_OFF_COLOR
                    : PIXEL_ON_COLOR;
                this.ctx.fillRect(col * PIXEL_SIZE, row * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
            }
        }
    };
    Display.prototype.getIndex = function (x, y) {
        return (y * this.width) + x;
    };
    return Display;
}());
export { Display };
