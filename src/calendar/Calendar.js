/**
 *  The frontend representation of the calendar in simple manner.
 *  
 *  This means, that this object is only a data holder, 
 *  it does not offer any of the functionality the backend offers.
 */
export class Calendar {

    /**
     * The title or alias assigned to a calendar to identify it in the gui.
     *
     *  @type {string}
     */
    name;

    /**
    *   A list of components that belong to this calendar.
    *   Each component is essentially just a list of properties too.
    *
    *   @type {Component[]}
    */
    components;

    /**
    *   A list of properties describing the calendar's characteristics.
    *
    *   @type {Record<string, string>[]}
    */
    properties;

    /**
    *   Initializes a new Calendar object.
    *
    *   @param {Component[][]} components belong to this calendar.
    *   @param {string[]} properties describe the calendar's characteristics.
    */
    constructor(name, components, properties) {
        this.name = name;
        this.components = components;
        this.properties = properties;
    }

    /**
     *  Map Calendar from the backend to this Calendar,
     *  which we can use in the frontend to render the calendar.
     *
     *  @param {Calendar[]} calendars an object containing the backend's calendar data.
     */
    static map(calendars) {
        let result = [];
        let components = [];
        let i = 0;

        const amount_of_calendars = calendars.length;

        // Loop calendars
        while (i < amount_of_calendars) {
            const calendar = calendars[i];
            const amountOfComponents = calendar.components.length;
            let j = 0;

            // Loop components
            while (j < amountOfComponents) {
                const component = calendar.components[j];
                let type;
                switch (component.type) {
                    case "Event": type = ComponentType.Event;
                    case "Todo": type = ComponentType.Todo;
                    case "Venue": type = ComponentType.Venue;
                    default: type = ComponentType.Other;
                };
                components.push(new Component(component.properties, type));
            }

            result.push(new Calendar(calendar.name, components, calendar.properties));
        }

        return result;
    }
}

class Component {

    /**
    *   A list of properties describing the component's characteristics.
    *
    *   @type {Record<string, string>[]}
    */
    properties;

    /**
    *   The type of Component
    *
    *   @type {ComponentType}
    */
    type;

    /**
    *   Initializes a new Component.
    *
    *   @param {string[]} properties describe the characteristics of the component.
    *   @param {ComponentType} type defines the type of Component.
    */
    constructor(properties, type) {
        this.properties = properties;
        this.type = type;
    }
}

// A Javascript enum for component types, each name is a color in hex format assigned.
const ComponentType = {
    Event: '#111111',
    Todo: '',
    Venue: '',
    Other: ''
}

Object.freeze(ComponentType);
