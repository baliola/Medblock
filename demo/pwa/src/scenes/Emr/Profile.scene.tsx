import React from 'react';
import {
  Book,
  CalendarDate,
  Home,
  Phone,
  Shield,
  StarRings,
  Text,
  UsersGroupRounded,
} from 'solar-icon-set';

import AppBar from '@/components/AppBar/AppBar';
import InfoItem from '@/components/mini/InfoItem';
import Images from '@/constants/images';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

const ProfilePage = () => {
  return (
    <Scaffold
      topBar={
        <AppBar
          title={
            <p className="text-lg text-gray-800 font-bold">
              Profile Information
            </p>
          }
        />
      }
    >
      <div className="p-6 mt-16">
        <div className="flex flex-col items-center">
          <img src={Images.dummyProfile} alt="" className="w-24" />
          <p className="text-gray-800 text-xl text-center mt-4 font-bold">
            {' '}
            I Wayan Arnijayadi Supatra
          </p>

          <div className="flex flex-row items-start space-x-2 my-2">
            <img src={Images.male} alt="" className="w-18" />
            <p className="text-gray-800">24 th</p>
          </div>
          <p>EMR ID: 123213213</p>
        </div>
        <div className="mt-6">
          <p className="text-gray-800 font-bold"> Peronal Information </p>
          <InfoItem
            label={'Home Address'}
            icon={<Home size={20} iconStyle="Bold" />}
            data="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
            classStyle="mt-4"
          />
          <InfoItem
            label={'Phone Number'}
            icon={<Phone size={20} iconStyle="Bold" />}
            data="+6289 213 476 271"
            classStyle="mt-4"
          />

          <InfoItem
            label={'Birthdate & Place'}
            icon={<CalendarDate size={20} iconStyle="Bold" />}
            data="Denpasar, 28 Mei 2000"
          />
        </div>
        <div className="mt-6  mb-64">
          <p className="text-gray-800 font-bold">Social Status</p>

          <InfoItem
            label={'Ethnicity'}
            icon={<UsersGroupRounded size={20} iconStyle="Bold" />}
            data="Balinese"
            classStyle="mt-4"
          />
          <InfoItem
            label={'Language'}
            icon={<Text size={20} iconStyle="Bold" />}
            data="Indonesia, English, Bali"
          />
          <InfoItem
            label={'Religion'}
            icon={<Book size={20} iconStyle="Bold" />}
            data="Hindu"
          />
          <InfoItem
            label={'Martial Status'}
            icon={<StarRings size={20} iconStyle="Bold" />}
            data="Maried"
          />
          <InfoItem
            label={'Partner'}
            icon={<Shield size={20} iconStyle="Bold" />}
            data="Ni Made Frastyasih Sumadi (Spouse)"
            subData="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
          />
          <InfoItem
            label={'Children'}
            icon={<UsersGroupRounded size={20} iconStyle="Bold" />}
            data="3 Daughter, 1 Son"
          />
        </div>
      </div>
    </Scaffold>
  );
};

export default ProfilePage;
