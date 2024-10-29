import { faker } from "@faker-js/faker";
import { NextResponse } from "next/server";

export async function GET(
  req: Request,
  { params }: { params: { id: string } }
) {
  const { id } = params;
    
  const data = {
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