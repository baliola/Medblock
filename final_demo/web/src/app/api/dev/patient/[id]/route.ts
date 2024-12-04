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
    personal_info: {
      home_address: faker.location.streetAddress(),
      phone: faker.phone.number(),
      date_of_birth: faker.date.birthdate(),
      place_of_birth: faker.location.city(),
      partner: {
        name: faker.person.fullName(),
        phone: faker.phone.number(),
        address: faker.location.streetAddress(),
        type: faker.helpers.arrayElement(
          ["husband", "wife", "partner", "son", "daughter" , "father", "mother"]
        )
      },
    }
  }

  return NextResponse.json({
    message: 'Success',
    data
  }, { status: 200 });
}