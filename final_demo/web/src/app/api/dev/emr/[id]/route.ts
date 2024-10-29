import { faker } from "@faker-js/faker";
import { NextResponse } from "next/server";

export async function GET(
  req: Request,
  { params }: { params: { id: string } }
) {
  const { id } = params;
    
  const data = {
    user_info: {
      avatar: faker.image.avatar(),
      full_name: faker.person.fullName(),
      age: faker.number.int({ min: 18, max: 60 }),
      sex: faker.person.sex(),
      id: id,
    },
    report: {
      overview: {
        hospital: faker.helpers.arrayElement([
          "Rumah Sakit Bross",
          "Rumah Sakit Sanglah",
          "Rumah Sakit Wangaya",
          "Rumah Sakit Bali Med",
          "Rumah Sakit Unud"
        ]),
        latest_visit: faker.date.recent(),
        medical_officer: faker.person.fullName(),
        vital_sign: {
          blood_pressure: faker.number.int({ min: 100, max: 200 }),
          temperature: faker.number.int({ min: 36, max: 40 }),
          heart_rate: faker.number.int({ min: 60, max: 100 }),
          respiration: faker.number.int({ min: 12, max: 20 }),
          oxygen_saturation: faker.number.int({ min: 90, max: 100 }),
        },
        visit_summary: {
          reason: faker.lorem.sentences(),
          diagnosis: faker.lorem.sentences(),
          planning: faker.lorem.sentences(),
          medication: faker.lorem.sentences(),
        }
      },
      notes: {
        id: faker.number.int(),
        date: faker.date.recent(),
        doctor: faker.person.fullName(),
        note: faker.lorem.paragraph(),
      },
      labs: Array.from({ length: 5 }).map(() => ({
        id: faker.number.int(),
        date: faker.date.recent(),
        lab: faker.lorem.sentence(),
        result: faker.lorem.sentence(),
      })),
      communication: {
        id: faker.number.int(),
        date: faker.date.recent(),
        doctor: faker.person.fullName(),
        note: faker.lorem.paragraph(),
      },
      imaging: {
        id: faker.number.int(),
        date: faker.date.recent(),
        imaging: faker.lorem.sentence(),
        result: faker.lorem.sentence(),
      },
      document: {
        id: faker.number.int(),
        date: faker.date.recent(),
        document: faker.lorem.sentence(),
        note: faker.lorem.sentence(),
      }
    },
    history: Array.from({ length: 8 }).map(() => ({
      id: faker.number.int(),
      hospital: {
        image: faker.image.urlPicsumPhotos(),
        name: faker.helpers.arrayElement([
          "Rumah Sakit Bross",
          "Rumah Sakit Sanglah",
          "Rumah Sakit Wangaya",
          "Rumah Sakit Bali Med",
          "Rumah Sakit Unud"
        ]),
      },
      doctor: faker.person.fullName(),
      date: faker.date.recent(),
      type: faker.helpers.arrayElement(
        ["Perawatan Lanjutan", "Rawat Inap", "Rawat Jalan"]
      ),
    })),
  }

  return NextResponse.json({
    message: 'Success',
    data
  }, { status: 200 });
}