import IconButton from '@components/button/IconButton';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';
import Images from '@constants/images';
import Strings from '@constants/strings';
import {styled} from 'nativewind';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {StyleProp, View, ViewStyle} from 'react-native';

const StyledView = styled(View);

interface ProfileBarProps {
  style?: StyleProp<ViewStyle>;
  trailingButton: React.ReactElement<any, any>;
  onPressTrailing: () => void;
}

const ProfileBar: React.FC<ProfileBarProps> = ({
  style,
  trailingButton,
  onPressTrailing,
}) => {
  const {t} = useTranslation('global');

  return (
    <StyledView
      className="flex flex-row justify-between m-6 items-center"
      style={style}>
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

      <IconButton onPress={onPressTrailing} icon={trailingButton} />
    </StyledView>
  );
};

export default ProfileBar;
