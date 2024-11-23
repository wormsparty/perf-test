import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';

import { EntityController } from './entity.controller';
import { EntityService } from './entity.service';
import { EntityEntity } from './entity.entity';

@Module({
  imports: [TypeOrmModule.forFeature([EntityEntity])],
  controllers: [EntityController],
  providers: [EntityService],
})
export class EntityModule {}
