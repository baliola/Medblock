import PrimaryButton from '@components/button/PrimaryButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {styled} from 'nativewind';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {StatusBar, View} from 'react-native';

const StyledView = styled(View);

const HomeScreen = () => {
  const {t} = useTranslation('global');

  return (
    <Scaffold
      topBar={
        <StyledView
          className="flex flex-row justify-between m-6 pt-4 items-center"
          style={{marginTop: StatusBar.currentHeight}}>
          <StyledView className="flex flex-row space-x-4 items-center">
            <GeneralImage size={50} url={Images.dummyProfile} />
            <StyledView className="flex flex-col items-start">
              <TextPrimary
                text="I Putu Aryadi"
                classStyle="text-gray-800 text-lg"
                isBold
              />
              <StyledView className="flex flex-row items-start space-x-2">
                <GeneralImage url={Images.male} size={18} />
                <TextPrimary text="24 th" classStyle="text-gray-800" isBold />
                <TextPrimary text="Maried" classStyle="text-gray-800" />
              </StyledView>
              <TextPrimary
                text={t(Strings.label.emrId, {
                  id: '234564213478',
                })}
              />
            </StyledView>
          </StyledView>

          <GeneralImage url={Images.notif} size={32} />
        </StyledView>
      }
      bottomChild={
        <StyledView className="p-6">
          <PrimaryButton
            child={
              <StyledView className="flex flex-row justify-center space-x-2 items-center">
                <GeneralImage url={Images.share} size={24} />
                <TextPrimary
                  text={Strings.label.shareCode}
                  classStyle="text-white"
                />
              </StyledView>
            }
            onPress={() => {}}
          />
        </StyledView>
      }>
      <StyledView className="flex-1 items-center px-6" />
    </Scaffold>
  );
};

export default HomeScreen;
