'use client';

import { useState } from 'react';
import { CalendarDate, Home, Logout, Mailbox, User } from 'solar-icon-set';

import AppBarWithIcon from '@/components/AppBar/AppBarWithIcon';
import SecondaryButton from '@/components/Button/SecondaryButton';
import DialogBasic from '@/components/Dialog/DialogBasic';
import InfoItem from '@/components/mini/InfoItem';
import Images from '@/constants/images';
import useAuthentication from '@/hooks/useAuth';
import { HomeLayout } from '@/layouts/HomeLayout/HomeLayout';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

export default function SettingPage() {
  const [open, setOpen] = useState<boolean>(false);
  const { signOut } = useAuthentication();

  return (
    <HomeLayout>
      <Scaffold
        topBar={<AppBarWithIcon />}
        bottomChild={
          <div className="p-6 mb-16">
            <SecondaryButton
              classStyle="bg-secondary-normal"
              child={
                <div className="flex flex-row justify-center space-x-2 items-center ">
                  <Logout color="white" size={18} iconStyle="Bold" />
                  <p>Logout</p>
                </div>
              }
              onSubmit={() => {
                setOpen(true);
              }}
            />
          </div>
        }
      >
        <div className="flex flex-col justify-center items-center mt-32">
          <img src={Images.dummyProfile} alt="" className="w-28" />
          <div className="w-full px-6 mt-10">
            <InfoItem
              data="I Wayan Aryadi"
              icon={<User size={20} iconStyle="Bold" />}
            />
            <InfoItem
              data="24 Mei 2000"
              icon={<CalendarDate size={20} iconStyle="Bold" />}
            />
            <InfoItem
              data="Arnijay@gmail.com"
              icon={<Mailbox size={20} iconStyle="Bold" />}
            />
            <InfoItem
              data="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
              icon={<Home size={20} iconStyle="Bold" />}
              classStyle="pr-8"
            />
          </div>
          <DialogBasic
            open={open}
            onCancel={() => {
              setOpen(false);
            }}
            onYes={() => {
              setOpen(false);
              signOut();
            }}
            title="Alert"
            labelCancel="Cancel"
            labelYes="Logout"
            child={
              <p className="text-sm text-gray-500">Are You Sure to quit?</p>
            }
          />
        </div>
      </Scaffold>
    </HomeLayout>
  );
}
