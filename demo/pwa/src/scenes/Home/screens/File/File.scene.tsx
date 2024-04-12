'use client';

import { HomeLayout } from '@/layouts/HomeLayout/HomeLayout';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

export default function FilePage() {
  return (
    <HomeLayout>
      <Scaffold>
        <p className="text-7xl">File</p>
      </Scaffold>
    </HomeLayout>
  );
}
