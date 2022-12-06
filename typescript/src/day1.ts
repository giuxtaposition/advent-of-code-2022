export const day1part1 = (caloriesList: string) =>
    greatestCalories(caloriesList, 1)

export const day1part2 = (caloriesList: string) =>
    greatestCalories(caloriesList, 3)

export const greatestCalories = (
    caloriesList: string,
    numberOfReindeers: number
) => {
    let sumList = caloriesList
        .split("\n\n")
        .map(reindeerCalories =>
            reindeerCalories
                .split("\n")
                .map(currentReindeerCalories =>
                    parseInt(currentReindeerCalories)
                )
                .reduce((previous, current) => previous + current)
        )
        .sort((a, b) => b - a)

    return sumList
        .slice(0, numberOfReindeers)
        .reduce((previous, current) => previous + current)
}
