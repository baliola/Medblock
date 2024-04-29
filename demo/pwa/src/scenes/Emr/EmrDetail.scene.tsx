import { useRouter } from 'next/router';
import { CloseCircle } from 'solar-icon-set';

import AppBar from '@/components/AppBar/AppBar';
import ProfileBar from '@/components/AppBar/ProfileBar';
import HeaderButton from '@/components/Button/HeaderButton';
import MetaItem from '@/components/mini/MetaItem';
import Images from '@/constants/images';
import useEMRPatient from '@/hooks/useEmrPatient';
import Scaffold from '@/layouts/ScaffoldLayout/ScafoldLayout';

import VisitSummaryItem from './components/VisitSummaryItem';

const EmrDetailPage = () => {
  const router = useRouter();
  const { emr } = useEMRPatient();

  return (
    <Scaffold
      topBar={
        <div className="flex flex-col bg-white">
          <AppBar title={<p className="text-lg text-gray-800">My EMR</p>} />
          <ProfileBar
            onPressTrailing={() => {
              router.push(`/emr/${emr?.header.emr_id}/revoke-access`);
            }}
            trailingButton={
              <div className="flex flex-col rounded-xl bg-secondary-light py-4 px-2 items-center w-20">
                <CloseCircle size={28} iconStyle="Bold" color="red" />
                <p className="text-secondary-normal text-center text-xs mt-1">
                  Close Access
                </p>
              </div>
            }
          />
        </div>
      }
    >
      <div className="px-6 mt-44 pb-44">
        <HeaderButton
          icon={Images.profile}
          label={'Profile Information'}
          onPress={() => {
            router.push(`/emr/${1}/profile`);
          }}
          classStyle="mb-6"
        />
        <HeaderButton
          icon={Images.hospital}
          label={
            emr
              ? (emr.body.find((fragment) => fragment.key === 'location')
                  ?.value as string)
              : ''
          }
          onPress={() => {}}
        />

        <div className="flex flex-row mt-8 justify-between items-center">
          <MetaItem data="27 March 2024" label={'Latest Visit'} />
          <MetaItem
            data={
              emr
                ? (emr.body.find((fragment) => fragment.key === 'doctor')
                    ?.value as string)
                : '-'
            }
            label={'Medical Officer'}
          />
        </div>

        <div className="mt-8 mb-60">
          <p className="text-gray-800 text-xl font-bold">Visit Summary</p>
          {emr?.body &&
            emr.body.map((item, index) => (
              <VisitSummaryItem
                key={index}
                label={item.key}
                data={item.value}
              />
            ))}
          {/* <VisitSummaryItem
            label={'Diagnosis'}
            data="Pemeriksaan fisik oleh dokter dan tes darah untuk memeriksa trombosit dan kadar hemoglobin serta tes serologi untuk mendeteksi virus dengue"
          />
          <VisitSummaryItem
            label={'Planning'}
            data="Istirahat yang cukup
Minum Banyak cairan
Mengkonsumsi obat penurun panas
Menghindari obat antiinflamasi
Rawat inap jika diperlukan"
          />
          <VisitSummaryItem
            label={'Medication'}
            data="Pengobatan simtomatik untuk meredakan demam, nyeri dan mual serta penggantian cairan intravena untuk mencegah dehidrasi, serta transfusi darah mungkon diperlukan"
          /> */}
        </div>
      </div>
    </Scaffold>
  );
};

export default EmrDetailPage;
