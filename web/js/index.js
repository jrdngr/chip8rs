// @ts-ignore
import { Cpu } from "../chip8";
// @ts-ignore
import { memory } from "../chip8_bg";
var cpu = Cpu.new();
function toArray(ptr) {
    return new Uint8Array(memory.buffer, ptr, 16);
}
var fileButton = document.getElementById("fileButton");
fileButton.onchange = function (event) {
    var file = event.target.files[0];
    var reader = new FileReader();
    reader.onload = function (args) {
        var arrayBuffer = args.target.result;
        var fileBytes = new Uint8Array(arrayBuffer);
        cpu.load_from_web(fileBytes);
        console.log("Loaded " + file.name);
    };
    reader.readAsArrayBuffer(file);
};
var cpuTable = document.createElement("table");
var cpuTableHeader = cpuTable.insertRow();
var cpuTableValues = cpuTable.insertRow();
var debugTable = {
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
var registerTable = document.createElement("table");
var registerNumbersRow = registerTable.insertRow();
var registerValuesRow = registerTable.insertRow();
var registerNumbers = [];
var registerValues = [];
for (var i = 0; i < 16; i++) {
    registerNumbers[i] = registerNumbersRow.insertCell();
    registerNumbers[i].innerHTML = "V" + i;
    registerValues[i] = registerValuesRow.insertCell();
}
registerTable.cellPadding = "5";
registerTable.border = "1px solid black";
document.body.appendChild(registerTable);
var stackTable = document.createElement("table");
var stackNumbersRow = stackTable.insertRow();
var stackValuesRow = stackTable.insertRow();
var stackNumbers = [];
var stackValues = [];
for (var i = 0; i < 16; i++) {
    stackNumbers[i] = stackNumbersRow.insertCell();
    stackNumbers[i].innerHTML = "S" + i;
    stackValues[i] = stackValuesRow.insertCell();
}
stackTable.cellPadding = "5";
stackTable.border = "1px solid black";
document.body.appendChild(stackTable);
var stepButton = document.createElement("button");
stepButton.innerText = "Step";
stepButton.addEventListener("click", stepCpu);
document.body.appendChild(stepButton);
document.addEventListener("keydown", function (event) {
    if (event.keyCode == 32) {
        stepCpu();
    }
});
function stepCpu() {
    cpu.step();
}
var REFRESH_RATE = 100;
function updateCpuValues() {
    debugTable.pcv.innerHTML = cpu.program_counter().toString();
    debugTable.spv.innerHTML = cpu.stack_pointer().toString();
    debugTable.dtv.innerHTML = cpu.delay_timer().toString();
    debugTable.stv.innerHTML = cpu.sound_timer().toString();
    var registers = toArray(cpu.data_registers());
    var stack = toArray(cpu.stack());
    for (var i = 0; i < 16; i++) {
        registerValues[i].innerHTML = registers[i].toString();
        stackValues[i].innerHTML = stack[i].toString();
    }
    setTimeout(updateCpuValues, REFRESH_RATE);
}
updateCpuValues();
var canvas = document.getElementById("canvas");
export function setPixel(x, y) {
}
export function clearScreen() {
}
export function getRandomSeed() {
}
