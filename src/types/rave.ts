export type Rave = {
    id: number;
    name: string;
    location: number;
    rating: number;
    date: string;
};

export function getRaveType(): Rave {
    let rave: Rave = {
        id: 0,
        name: "",
        location: 0,
        rating: 0,
        date: "1970-01-01",
    };

    return rave;
}
