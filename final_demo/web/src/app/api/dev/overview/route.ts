import { faker } from "@faker-js/faker";
import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const data = {
    hospital: {
      name: faker.helpers.arrayElement([
        "Rumah Sakit Bross",
        "Rumah Sakit Sanglah",
        "Rumah Sakit Wangaya",
        "Rumah Sakit Bali Med",
        "Rumah Sakit Unud"
      ]),
      id: faker.number.int(),
      image: faker.image.urlPicsumPhotos(),
    },
    patients: faker.number.int({ min: 1000, max: 400000 }),
    emr: faker.number.int({ min: 1000, max: 400000 }),
    doctors: faker.number.int({ min: 1000, max: 400000 }),
    patient_gender: {
      male: faker.number.int({ min: 1000, max: 400000 }),
      female: faker.number.int({ min: 1000, max: 400000 }),
    },
    patient_age: {
      "0-18": faker.number.int({ min: 1000, max: 400000 }),
      "19-30": faker.number.int({ min: 1000, max: 400000 }),
      "31-45": faker.number.int({ min: 1000, max: 400000 }),
      "46-60": faker.number.int({ min: 1000, max: 400000 }),
      "60+": faker.number.int({ min: 1000, max: 400000 }),
    },
    polyclinics: Array.from({ length: 5 }).map(() => ({
      id: faker.number.int(),
      name: faker.helpers.arrayElement([
        "Internal",
        "Other",
        "Psychiatry",
        "Orthopedy",
        "Dental"
      ]),
      patients: faker.number.int({ min: 1000, max: 400000 }),
      doctors: faker.number.int({ min: 1000, max: 400000 }),
      emr: faker.number.int({ min: 1000, max: 400000 }),
    })),
  }
  
  return NextResponse.json({
    message: 'Success',
    data
  }, { status: 200 });
}