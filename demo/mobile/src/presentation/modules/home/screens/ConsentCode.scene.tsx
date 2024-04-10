import BasicButton from '@components/button/BasicButton';
import TextButton from '@components/button/TextButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Colors from '@constants/colors';
import Images from '@constants/images';
import Strings from '@constants/strings';
import Scaffold from '@layouts/Scaffold';
import {useNavigation} from '@react-navigation/native';
import {styled} from 'nativewind';
import React, {useEffect, useState} from 'react';
import {useTranslation} from 'react-i18next';
import {View} from 'react-native';

const StyledView = styled(View);

const ConsentCodeScreen = () => {
  const {t} = useTranslation('global');
  const [seconds, setSeconds] = useState<number>(59);
  const navigation = useNavigation();

  useEffect(() => {
    let intervalId: NodeJS.Timeout;

    if (seconds > 0) {
      intervalId = setInterval(() => {
        setSeconds(prevSeconds => prevSeconds - 1);
      }, 1000);
    }

    return () => {
      clearInterval(intervalId);
    };
  }, [seconds]);

  const handleResendCode = () => {
    setSeconds(59);
  };

  const handleBack = () => {
    navigation.goBack();
  };

  return (
    <Scaffold background={Colors.primary_normal}>
      <StyledView className="relative flex flex-col h-screen items-center px-4 mt-40">
        <TextPrimary
          text={Strings.message.shareConsentCode}
          classStyle="text-white text-2xl text-center"
          isBold
        />
        <TextPrimary
          text="613487"
          classStyle="text-white text-7xl my-6"
          isBold
        />
        <TextPrimary
          text={t(Strings.label.refreshCode, {
            second: seconds,
          })}
          classStyle="text-white"
        />

        <TextButton
          label={Strings.label.getNewCode}
          onPress={() => {
            handleResendCode();
          }}
          classStyle="underline text-yellow-500 my-6"
        />

        <BasicButton
          label={Strings.label.back}
          labelStyle={{color: Colors.primary_light}}
          onPress={() => {
            handleBack();
          }}
          classStyle="bg-white rounded-xl py-4 px-12"
        />
        <GeneralImage
          url={Images.consentCode}
          size={520}
          classStyle="absolute -bottom-10"
        />
      </StyledView>
    </Scaffold>
  );
};

export default ConsentCodeScreen;
