export type Conditions = {
    get_upcoming: boolean;
    get_previous: boolean;
    get_all: boolean;
    limit: number;
    offset: number;
    get_artists: boolean;
    get_expenses: boolean;
    get_locations: boolean;
    get_ratings: boolean;
};

export function getConditionsType(): Conditions {
    let conditions: Conditions = {
        get_upcoming: false,
        get_previous: false,
        get_all: false,
        limit: 0,
        offset: 0,
        get_artists: false,
        get_expenses: false,
        get_locations: false,
        get_ratings: false,
    };

    return conditions;
}
