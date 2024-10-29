import { faker } from "@faker-js/faker";
import { NextResponse } from "next/server";

export async function GET(req: Request) {
  const { searchParams } = new URL(req.url);

  const page = Number(searchParams.get('page')) || 1;
  const search = searchParams.get('search') || '';
  const limit = Number(searchParams.get('limit')) || 10;
  
  const totalData = 200;
  const totalPages = Math.ceil(totalData / limit);
  
  const datas = Array.from({ length: limit }).map(() => ({
    id: faker.number.int(),
    full_name: faker.person.fullName(),
    date_of_birth: faker.date.birthdate(),
    place_of_birth: faker.location.city(),
    address: faker.location.streetAddress(),
  }));

  return NextResponse.json({
    message: 'Success',
    data: datas,
    metadata: {
      totalData,
      totalPages,
      currentPage: page,
      limit
    }
  }, { status: 200 });
}