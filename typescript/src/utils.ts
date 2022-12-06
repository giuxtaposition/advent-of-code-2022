import { readFileSync } from "fs"
import path, { join } from "path"

export function readFile(filename: string): string {
    let docsDir = path.resolve("./").replace("typescript", "days_docs")
    return readFileSync(join(docsDir, filename), "utf-8")
}
