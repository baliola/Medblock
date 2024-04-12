'use client';

import { HomeLayout } from '@/layouts/HomeLayout/HomeLayout';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

export default function SettingPage() {
  return (
    <HomeLayout>
      <Scaffold>
        <p className="text-7xl">Setting</p>
      </Scaffold>
    </HomeLayout>
  );
}
