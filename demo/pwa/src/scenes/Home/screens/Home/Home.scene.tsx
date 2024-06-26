'use client';

import { useRouter } from 'next/router';
import { useEffect } from 'react';
import { File, Share } from 'solar-icon-set';

import AppBarWithIcon from '@/components/AppBar/AppBarWithIcon';
import ProfileBar from '@/components/AppBar/ProfileBar';
import PrimaryButton from '@/components/Button/PrimaryButton';
import Images from '@/constants/images';
import useEMRPatient from '@/hooks/useEmrPatient';
import usePatient from '@/hooks/usePatient';
import { HomeLayout } from '@/layouts/HomeLayout/HomeLayout';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

export default function HomePage() {
  const router = useRouter();
  const data = [{ id: 1, title: 'My EMR' }];

  const { shareConsetCode, loading: loadingConsentCode } = usePatient();
  const { emrList, loading } = useEMRPatient();

  // useEffect(() => {
  //   getPatientInfo();
  //   console.log('====================================');
  //   console.log('DATA INGBOX ===>> ', patientInfo);
  //   console.log('====================================');
  // }, []);

  return (
    <HomeLayout>
      <Scaffold
        loading={loading || loadingConsentCode}
        topBar={
          <AppBarWithIcon
            child={
              <ProfileBar
                onPressTrailing={() => {
                  router.push('/home/notification');
                }}
                trailingButton={
                  <img src={Images.notif} alt="" className="w-8" />
                }
              />
            }
          />
        }
        bottomChild={
          <div className="p-6 mb-16">
            <PrimaryButton
              child={
                <div className="flex flex-row justify-center space-x-2 items-center">
                  <Share color="white" size={18} iconStyle="Bold" />
                  <p>Share Code</p>
                </div>
              }
              onSubmit={() => {
                shareConsetCode();
              }}
            />
          </div>
        }
      >
        <div className="mt-48 px-6">
          <div className="grid grid-cols-3 gap-4">
            {emrList.map((item) => {
              return (
                <div
                  onClick={() => {
                    // router.push(`/emr/${item.emr_id}`);
                    router.push({
                      pathname: `/emr/${item.header.emr_id}`,
                      query: {
                        providerId: item.header.provider_id,
                      },
                    });
                  }}
                  key={item.header.emr_id}
                  className="flex flex-col items-center bg-slate-200 rounded-xl p-4"
                >
                  <File color="#242DA8" size={18} iconStyle="Bold" />
                  <p className="text-gray-800 mt-2 text-sm">
                    {' '}
                    {item.header.emr_id.substring(0, 5) +
                      '...' +
                      item.header.emr_id.substring(31, 36)}{' '}
                  </p>
                  <p className="text-gray-800 mt-2 text-sm"> </p>
                </div>
              );
            })}
          </div>
        </div>
      </Scaffold>
    </HomeLayout>
  );
}
