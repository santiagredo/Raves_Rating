export type Location = {
    id: number;
    name: string;
    address: string;
    capacity: string;
};

export default function getLocationType(): Location {
    let location = {
        id: 0,
        name: "",
        address: "",
        capacity: "",
    };

    return location;
}
