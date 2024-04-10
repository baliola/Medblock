import GeneralImage from '@components/image/GeneralImage';
import InfoItem from '@components/partial/InfoItem';
import TextPrimary from '@components/text/TextPrimary';
import AppBar from '@components/top_bar/AppBar';
import Images from '@constants/images';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {ScrollView, View} from 'react-native';

const StyledView = styled(View);
const StyledScrollView = styled(ScrollView);

const ProfileInformationScreen = () => {
  const {t} = useTranslation('global');

  return (
    <Scaffold
      topBar={
        <AppBar
          title={
            <TextPrimary
              text={Strings.label.profileInfo}
              classStyle="text-lg text-gray-800"
              isBold={true}
            />
          }
        />
      }>
      <StyledScrollView className="p-6" showsVerticalScrollIndicator={false}>
        <StyledView className="flex flex-col items-center">
          <GeneralImage size={100} url={Images.dummyProfile} />
          <TextPrimary
            text="I Wayan Arnijayadi Supatra"
            classStyle="text-gray-800 text-xl text-center mt-4"
            isBold
          />
          <StyledView className="flex flex-row items-start space-x-2 my-2">
            <GeneralImage url={Images.male} size={18} />
            <TextPrimary text="24 th" classStyle="text-gray-800" />
          </StyledView>
          <TextPrimary
            text={t(Strings.label.emrId, {
              id: '234564213478',
            })}
          />
        </StyledView>
        <StyledView className="mt-6">
          <TextPrimary
            text={Strings.label.personalInfo}
            classStyle="text-gray-800"
            isBold
          />
          <InfoItem
            label={Strings.label.homeAddress}
            icon={Images.address}
            data="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
            classStyle="mt-4"
          />
          <InfoItem
            label={Strings.label.phoneNumber}
            icon={Images.phone}
            data="+6289 213 476 271"
          />
          <InfoItem
            label={Strings.label.birthDate}
            icon={Images.birthDate}
            data="Denpasar, 28 Mei 2000"
          />
        </StyledView>
        <StyledView className="mt-6  mb-64">
          <TextPrimary
            text={Strings.label.socialStatus}
            classStyle="text-gray-800"
            isBold
          />
          <InfoItem
            label={Strings.label.ethnicity}
            icon={Images.ethnic}
            data="Balinese"
            classStyle="mt-4"
          />
          <InfoItem
            label={Strings.label.language}
            icon={Images.language}
            data="Indonesia, English, Bali"
          />
          <InfoItem
            label={Strings.label.religion}
            icon={Images.hindu}
            data="Hindu"
          />
          <InfoItem
            label={Strings.label.martialStatus}
            icon={Images.martialStatus}
            data="Maried"
          />
          <InfoItem
            label={Strings.label.partner}
            icon={Images.guard}
            data="Ni Made Frastyasih Sumadi (Spouse)"
            subData="Jl. Sangalangit Gang Merpati No.46, Denpasar, Bali"
          />
          <InfoItem
            label={Strings.label.children}
            icon={Images.tedy}
            data="3 Daughter, 1 Son"
          />
        </StyledView>
      </StyledScrollView>
    </Scaffold>
  );
};

export default ProfileInformationScreen;
