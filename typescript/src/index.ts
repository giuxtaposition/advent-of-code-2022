import { day1part1, day1part2 } from "./day1"
import { readFile } from "./utils"

const adventOfCode = () => {
    console.log("🎄 Advent Of Code 2022 🎄")
    console.log(`day1 part1: ${day1part1(readFile("day1/input.txt"))}`)
    console.log(`day1 part2: ${day1part2(readFile("day1/input.txt"))}`)
}

adventOfCode()
