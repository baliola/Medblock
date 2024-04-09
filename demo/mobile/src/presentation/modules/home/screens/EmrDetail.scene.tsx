import HeaderButton from '@components/button/HeaderButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import AppBar from '@components/top_bar/AppBar';
import ProfileBar from '@components/top_bar/ProfileBar';
import Images from '@constants/images';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React from 'react';
import {ScrollView, View} from 'react-native';
import MetaItem from '../components/MetaItem';
import VisitSummaryItem from '../components/VisitSummaryItem';
import {useNavigation} from '@react-navigation/native';
import {NativeStackNavigationProp} from '@react-navigation/native-stack';
import {RootStackParamList} from '@constants/routes';

const StyledView = styled(View);
const StyledScrollView = styled(ScrollView);

const EmrDetailScreen = () => {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();

  return (
    <Scaffold
      topBar={
        <StyledView className="flex flex-col">
          <AppBar
            title={
              <TextPrimary
                text={'My EMR'}
                classStyle="text-lg text-gray-800"
                isBold={true}
              />
            }
          />
          <ProfileBar
            onPressTrailing={() => {
              navigation.navigate('RevokeAccess');
            }}
            trailingButton={
              <StyledView className="flex flex-col rounded-xl bg-secondary-light py-3 px-2 items-center w-20">
                <GeneralImage url={Images.close} size={28} />
                <TextPrimary
                  text={Strings.label.closeAccess}
                  classStyle="text-secondary-normal text-center text-xs mt-1"
                />
              </StyledView>
            }
          />
        </StyledView>
      }>
      <StyledScrollView className="px-6" showsVerticalScrollIndicator={false}>
        <HeaderButton
          icon={Images.profile}
          label={Strings.label.profileInfo}
          onPress={() => {
            navigation.navigate('ProfileInformation');
          }}
          classStyle="mb-6"
        />
        <HeaderButton
          icon={Images.hospital}
          label={'Sanglah Hospital - Denpasar'}
          onPress={() => {}}
        />

        <StyledView className="flex flex-row mt-8 justify-between items-center">
          <MetaItem data="27 March 2024" label={Strings.label.latestVisit} />
          <MetaItem
            data="Karyada Indrawan"
            label={Strings.label.medicalOfficer}
          />
        </StyledView>

        <StyledView className="mt-8 mb-60">
          <TextPrimary
            text={Strings.label.visitSummary}
            classStyle="text-gray-800 text-xl"
            isBold
          />

          <VisitSummaryItem
            label={Strings.label.reasonVisit}
            data="Mengalami demam tinggi (diatas 38°C) mengalami nyeri kepala, otot dan sendi serta mengalami ruam kulit. Setiap makan malam selalu mual dan muntah, mudah merasa kelelahan"
          />
          <VisitSummaryItem
            label={Strings.label.diagnosis}
            data="Pemeriksaan fisik oleh dokter dan tes darah untuk memeriksa trombosit dan kadar hemoglobin serta tes serologi untuk mendeteksi virus dengue"
          />
          <VisitSummaryItem
            label={Strings.label.reasonVisit}
            data="Istirahat yang cukup
Minum Banyak cairan
Mengkonsumsi obat penurun panas
Menghindari obat antiinflamasi
Rawat inap jika diperlukan"
          />
          <VisitSummaryItem
            label={Strings.label.medication}
            data="Pengobatan simtomatik untuk meredakan demam, nyeri dan mual serta penggantian cairan intravena untuk mencegah dehidrasi, serta transfusi darah mungkon diperlukan"
          />
        </StyledView>
      </StyledScrollView>
    </Scaffold>
  );
};

export default EmrDetailScreen;
