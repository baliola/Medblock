import React from 'react';
import BasicButton from './BasicButton';
import {styled} from 'nativewind';
import {View} from 'react-native';
import GeneralImage from '@components/image/GeneralImage';
import TextPrimary from '@components/text/TextPrimary';

const StyledView = styled(View);

interface IconLabelButtonProps {
  label: string;
  icon: string;
  onPress: () => void;
  classStyle?: string;
}

const IconLabelButton: React.FC<IconLabelButtonProps> = ({
  icon,
  label,
  onPress,
  classStyle,
}) => {
  return (
    <BasicButton
      classStyle={classStyle}
      onPress={() => {
        onPress();
      }}
      child={
        <StyledView className="flex flex-row items-center space-x-2">
          <GeneralImage url={icon} size={24} />
          <TextPrimary text={label} classStyle="text-gray-800" />
        </StyledView>
      }
    />
  );
};

export default IconLabelButton;
