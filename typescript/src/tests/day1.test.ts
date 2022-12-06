import { expect, test, describe } from "vitest"
import { greatestCalories } from "../day1"

describe("Day 1", () => {
    test("get calories from reindeer with most calories", () => {
        let caloriesList =
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"

        expect(greatestCalories(caloriesList, 1)).toEqual(24000)
    })

    test("get calories from top 3 reindeers with most calories", () => {
        let caloriesList =
            "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"

        expect(greatestCalories(caloriesList, 3)).toEqual(45000)
    })
})
