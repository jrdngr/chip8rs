export class Keyboard {
    
    private readonly keyStates: boolean[] = [];
   
    constructor() {
        for (let i = 0; i < 16; i ++) {
            this.keyStates[i] = false;
        }

        document.addEventListener("keydown", this.onKeyDown.bind(this));
        document.addEventListener("keyup", this.onKeyUp.bind(this));
    }

    public isKeyDown(key: number): boolean {
        if (key < 0 || key >= 16) {
            return false;
        }
        return this.keyStates[key];
    }

    public getAnyKey(): number {
        for (let i = 0; i < this.keyStates.length; i++) {
            if (this.keyStates[i]) {
                return i;
            }
        }
        return -1;
    }

    private onKeyDown(event: KeyboardEvent) {
        this.setKeyState(parseInt(event.key), true);
    }

    private onKeyUp(event: KeyboardEvent) {
        this.setKeyState(parseInt(event.key), false);
    }

    private setKeyState(key: number, isDown: boolean) {
        if (key < 16 && key >= 0) {
            this.keyStates[key] = true;
        }
    }
}