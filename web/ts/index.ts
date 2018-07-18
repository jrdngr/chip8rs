// @ts-ignore
import { Cpu } from "../chip8";
// @ts-ignore
import { memory } from "../chip8_bg";

const cpu = Cpu.new();

function toArray(ptr: number) {
    return new Uint8Array(memory.buffer, ptr, 16);
}


const fileButton = document.getElementById("fileButton");
fileButton.onchange = function(event: any) {
    const file = event.target.files[0];
    const reader = new FileReader();
    reader.onload = function(args) {
        var arrayBuffer = args.target.result;
        var fileBytes = new Uint8Array(arrayBuffer);
        cpu.load_from_web(fileBytes);
        console.log("Loaded " + file.name);
    };
    reader.readAsArrayBuffer(file);
}

const cpuTable = document.createElement("table");
const cpuTableHeader = cpuTable.insertRow();
const cpuTableValues = cpuTable.insertRow();
const debugTable = {
    table: cpuTable,
    header: cpuTableHeader,
    pc: cpuTableHeader.insertCell().innerHTML = "PC",
    sp: cpuTableHeader.insertCell().innerHTML = "SP",
    dt: cpuTableHeader.insertCell().innerHTML = "DT",
    st: cpuTableHeader.insertCell().innerHTML = "ST",
    values: cpuTableValues,
    pcv: cpuTableValues.insertCell(),
    spv: cpuTableValues.insertCell(),
    dtv: cpuTableValues.insertCell(),
    stv: cpuTableValues.insertCell(),
};

cpuTable.cellPadding = "5";
cpuTable.border = "1px solid black";
document.body.appendChild(debugTable.table);


const registerTable = document.createElement("table");
const registerNumbersRow = registerTable.insertRow();
const registerValuesRow = registerTable.insertRow();

const registerNumbers = [];
const registerValues: HTMLTableDataCellElement[] = [];
for (let i = 0; i < 16; i++) {
    registerNumbers[i] = registerNumbersRow.insertCell();
    registerNumbers[i].innerHTML = "V" + i;
    registerValues[i] = registerValuesRow.insertCell();
}

registerTable.cellPadding = "5";
registerTable.border = "1px solid black";
document.body.appendChild(registerTable);



const stackTable = document.createElement("table");
const stackNumbersRow = stackTable.insertRow();
const stackValuesRow = stackTable.insertRow();

const stackNumbers = [];
const stackValues: HTMLTableDataCellElement[] = [];
for (let i = 0; i < 16; i++) {
    stackNumbers[i] = stackNumbersRow.insertCell();
    stackNumbers[i].innerHTML = "S" + i;
    stackValues[i] = stackValuesRow.insertCell();
}

stackTable.cellPadding = "5";
stackTable.border = "1px solid black";
document.body.appendChild(stackTable);



const stepButton = document.createElement("button");
stepButton.innerText = "Step";
stepButton.addEventListener("click", () => cpu.step());
document.body.appendChild(stepButton);


const REFRESH_RATE = 100;

function updateCpuValues() {
    debugTable.pcv.innerHTML = cpu.program_counter().toString();
    debugTable.spv.innerHTML = cpu.stack_pointer().toString();
    debugTable.dtv.innerHTML = cpu.delay_timer().toString();
    debugTable.stv.innerHTML = cpu.sound_timer().toString();

    const registers = toArray(cpu.data_registers());
    const stack = toArray(cpu.stack());

    for (let i = 0; i < 16; i++) {
        registerValues[i].innerHTML = registers[i].toString();
        stackValues[i].innerHTML = stack[i].toString();
    }

    setTimeout(updateCpuValues, REFRESH_RATE);
}

updateCpuValues();

const canvas = <HTMLCanvasElement>document.getElementById("canvas");

export function setPixel(x: number, y: number) {
    console.log(x + " " + y);
}

export function clearScreen() {
    console.log("clear");
}

export function getRandomSeed() {
    
}