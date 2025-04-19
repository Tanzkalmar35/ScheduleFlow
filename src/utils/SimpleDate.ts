export class IDate {
	private readonly date: Date;

	private constructor(date: Date) {
		this.date = date;
	}

	/**
	 * Parses a string that has the correct format into an IDate object.
	 *
	 * @static
	 * @param {string} dateString - The string of the date. 
	 * Has to have exactly the following format: "yyyy:mm:dd hh:mm:ss digital_time", so for example "2024-10-20 18:04:16.213712372 +02:00"
	 * @throws {Error} - If the date string is incorrectly formatted.
	 * @returns {IDate} The parsed IDate object.
	 */
	static parseString(dateString: string): IDate {
		if (!IDate.isFormat(dateString)) {
			console.log(dateString)
			throw new Error("Invalid date string format given for parsing.")
		}

		const splitDate = dateString.split(" ")[0];
		const splitTime = dateString.split(" ")[1];

		const date = new Date(
			Number(splitDate.split("-")[0]),
			Number(splitDate.split("-")[1]),
			Number(splitDate.split("-")[2]),
			Number(splitTime.split(":")[0]),
			Number(splitTime.split(":")[1]),
			Number(splitTime.split(":")[2]),
			0
		)

		return new IDate(date);
	}

	/**
	 * Validates the format of a given datetime string.
	 *
	 * @static
	 * @param {string} dateString - The given datetime string checked for correct formatting.
	 * @returns {boolean} true, if the given string conforms the format, otherwise false.
	 */
	private static isFormat(dateString: string): boolean {
		let dateTimeSplit = dateString.split(" ");

		if (dateTimeSplit.length !== 3) return false;
		if (!IDate.isDateFormat(dateTimeSplit[0])) return false;
		if (!IDate.isTimeFormat(dateTimeSplit[1])) return false;

		return true;
	}

	/**
	 * Validates the formatting of the date portion of the datetime string.
	 *
	 * @static
	 * @param {string} date - The date portion of the datetime string given for formatting verification.
	 * @returns {boolean} true, if the given string conforms the format, otherwise false.
	 */
	private static isDateFormat(date: string): boolean {
		const dateSplit = date.split("-");


		if (dateSplit.length !== 3) return false;

		dateSplit.forEach(split => {
			if (isNaN(Number(split))) {
				return false;
			}
		})

		return true;
	}

	/**
	 * Validates the formatting of the time portion of the datetime string.
	 *
	 * @static
	 * @param {string} time - The time string given for a formatting check.
	 * @returns {boolean} true, if the given string conforms the format, otherwise false.
	 */
	private static isTimeFormat(time: string): boolean {
		const timeSplit = time.split(":");

		if (timeSplit.length !== 3) return false;

		timeSplit.forEach(split => {
			if (isNaN(Number(split))) return false;
		})

		return true;
	}

	/**
	 * Compares two datetimes.
	 *
	 * @param {IDate} other - The other datetime to compare to.
	 * @returns {number} -1, if this datetime is older than the given. 0, if equal, and 1 if the other datetime is older than this one.
	 */
	compareTo(other: IDate): number {
		// Compare years
		if (this.date.getFullYear() < other.date.getFullYear()) {
			console.log()
			return -1;
		} else if (this.date.getFullYear() > other.date.getFullYear()) {
			return 1;
		}

		// Compare months
		if (this.date.getMonth() < other.date.getMonth()) {
			return -1;
		} else if (this.date.getMonth() > other.date.getMonth()) {
			return 1;
		}

		// Compare days
		if (this.date.getDay() < other.date.getDay()) {
			return -1;
		} else if (this.date.getDay() > other.date.getDay()) {
			return 1;
		}

		// Compare hours
		if (this.date.getHours() < other.date.getHours()) {
			return -1;
		} else if (this.date.getHours() > other.date.getHours()) {
			return 1;
		}

		// Compare minutes
		if (this.date.getMinutes() < other.date.getMinutes()) {
			return -1;
		} else if (this.date.getMinutes() > other.date.getMinutes()) {
			return 1;
		}

		// Compare seconds
		if (this.date.getSeconds() < other.date.getSeconds()) {
			return -1;
		} else if (this.date.getSeconds() > other.date.getSeconds()) {
			return 1;
		}

		// Compare milliseconds
		if (this.date.getMilliseconds() < other.date.getMilliseconds()) {
			return -1;
		} else if (this.date.getMilliseconds() > other.date.getMilliseconds()) {
			return 1;
		}

		// Two datetimes are exactly equal
		return 0;
	}

	/**
	 * Increases this date by one day.
	 *
	 * @returns {IDate} An IDate object one day in advance from this one.
	 */
	coypIncreaseOneDay(): IDate {
		this.date.setDate(this.date.getDate() + 1)
		return this;
	}
}
