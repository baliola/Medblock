import {styled} from 'nativewind';
import React from 'react';
import {useTranslation} from 'react-i18next';
import {Text, TextStyle} from 'react-native';

const StyledText = styled(Text);

interface TextPrimaryProps {
  text?: string;
  classStyle?: string;
  style?: TextStyle;
  isBold?: boolean;
}

const TextPrimary: React.FC<TextPrimaryProps> = ({
  classStyle,
  text,
  style,
  isBold,
}) => {
  const {t} = useTranslation('global');

  let fontFamily = 'Ubuntu-Regular';

  if (isBold) {
    fontFamily = 'Ubuntu-Bold';
  }

  return (
    <StyledText
      className={classStyle}
      style={[style, {fontFamily: fontFamily}]}>
      {t(text as string)}
    </StyledText>
  );
};

export default TextPrimary;
