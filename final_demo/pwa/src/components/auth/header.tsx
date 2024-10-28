import { LOGO } from "@/constants/logo";
import { Flex, Image, Text } from "@chakra-ui/react";

interface AuthHeaderProps {
  size?: 'xs' | 'sm' | 'lg';
}

const headerVariants = {
  xs: {
    imageWidth: 16,
    titleFontSize: 'xl',
    titleFontWeight: 'bold',
    subtitleFontSize: 'sm',
    subtitleFontWeight: 'normal',
  },
  sm: {
    imageWidth: 20,
    titleFontSize: '2xl',
    titleFontWeight: 'bold',
    subtitleFontSize: 'md',
    subtitleFontWeight: 'normal',
  },
  lg: {
    imageWidth: 24,
    titleFontSize: '4xl',
    titleFontWeight: 'bold',
    subtitleFontSize: 'xl',
    subtitleFontWeight: 'bold',
  },
};

export const AuthHeader = ({
  size = "lg"
}: AuthHeaderProps) => {
  const variant = headerVariants[size]

  return (
    <Flex as="header"
      direction={'column'}
      align={'center'}
      gap={0}
      mb={5}
    >
      <Image
        src={LOGO}
        alt="Medblock Passport"
        w={variant.imageWidth}
        mb={size === "lg" ? 5 : 0}
      />
      <Text as="h1"
        color={"neutral.700"}
        fontWeight={variant.titleFontWeight}
        fontSize={variant.titleFontSize}
      >
        Med
        <Text as="span" color="accent.700">
          block
        </Text>
      </Text>
      <Text as="p"
        fontSize={variant.subtitleFontSize}
        color={"neutral.700"}
        fontWeight={variant.subtitleFontWeight}
      >
        Passport
      </Text>
    </Flex>
  )
}